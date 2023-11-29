#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn add_workflow_test_pass() {
        let composer = Composer::default();

        let workflow1 = Workflow {
            name: "test-workflow".to_string(),
            version: "0.0.1".to_string(),
            tasks: HashMap::default(),
            custom_types: None,
        };

        composer
            .add_workflow(
                "test-workflow".to_string(),
                "0.0.1".to_string(),
                HashMap::default(),
                None,
            )
            .unwrap();

        let composer_workflow = &composer.workflows.borrow()[0];

        assert_eq!(composer_workflow, &workflow1);
    }

    #[test]
    fn get_dependencies_test() {
        let composer = Composer::default();

        let mut dependencies : Vec<Depend> = Vec::new();
        dependencies.push(Depend { task_name: "dependent_task".to_string(), cur_field: "id".to_string(), prev_field: "ids".to_string() });

        let task = Task {
            action_name: "get_salaries".to_string(),
            depend_on: dependencies,
            ..Default::default()
        };

        let mut tasks = HashMap::<String, Task>::new();
        tasks.insert("get_salaries".to_string(), task);

        composer
            .add_workflow(
                "test-workflow".to_string(),
                "0.0.1".to_string(),
                tasks,
                None,
            )
            .unwrap();

        assert_eq!(
            composer.get_dependencies("get_salaries", 0).unwrap(),
            vec!["dependent_task"]
        );
    }

    #[test]
    fn get_flow_test() {
        let composer = Composer::default();

        let task0 = Task {
            action_name: "task0".to_string(),
            ..Default::default()
        };
        let mut task1 = Task {
            action_name: "task1".to_string(),
            ..Default::default()
        };

        let mut dependencies : Vec<Depend> = Vec::new();
        dependencies.push(Depend { task_name: "task0".to_string(), cur_field: "id".to_string(), prev_field: "ids".to_string() });
        dependencies.push(Depend { task_name: "task4".to_string(), cur_field: "id".to_string(), prev_field: "ids".to_string() });
        task1.depend_on = dependencies;

        let mut task2 = Task {
            action_name: "task2".to_string(),
            ..Default::default()
        };
    
        let mut dependencies : Vec<Depend> = Vec::new();
        dependencies.push(Depend { task_name: "task0".to_string(), cur_field: "id".to_string(), prev_field: "ids".to_string() });
        task2.depend_on = dependencies;

        let mut task3 = Task {
            action_name: "task3".to_string(),
            ..Default::default()
        };
       
        let mut dependencies : Vec<Depend> = Vec::new();
        dependencies.push(Depend { task_name: "task1".to_string(), cur_field: "id".to_string(), prev_field: "ids".to_string() });
        dependencies.push(Depend { task_name: "task2".to_string(), cur_field: "id".to_string(), prev_field: "ids".to_string() });
        task3.depend_on = dependencies;

        let task4 = Task {
            action_name: "task4".to_string(),
            ..Default::default()
        };
        let mut task5 = Task {
            action_name: "task5".to_string(),
            ..Default::default()
        };

        let mut dependencies : Vec<Depend> = Vec::new();
        dependencies.push(Depend { task_name: "task2".to_string(), cur_field: "id".to_string(), prev_field: "ids".to_string() });
        task5.depend_on = dependencies;

        let mut tasks = HashMap::new();
        tasks.insert("task0".to_string(), task0);
        tasks.insert("task1".to_string(), task1);
        tasks.insert("task2".to_string(), task2);
        tasks.insert("task3".to_string(), task3);
        tasks.insert("task4".to_string(), task4);
        tasks.insert("task5".to_string(), task5);

        composer
            .add_workflow(
                "test-workflow".to_string(),
                "0.0.1".to_string(),
                tasks,
                None,
            )
            .unwrap();

        let flow = composer.get_flow(0);

        assert!(flow[0] == "task0" || flow[0] == "task4");

        assert!(flow[1] == "task0" || flow[1] == "task4" || flow[1] == "task2");

        assert!(
            flow[2] == "task1" || flow[2] == "task2" || flow[2] == "task4" || flow[2] == "task5"
        );

        assert!(
            flow[3] == "task1" || flow[3] == "task2" || flow[3] == "task4" || flow[3] == "task5"
        );

        assert!(flow[4] == "task3" || flow[4] == "task5" || flow[4] == "task1");

        assert!(flow[5] == "task3" || flow[5] == "task5");
    }

    #[test]
    fn get_attributes_test() {
        let composer = Composer::default();

        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert("namespace".to_string(), "value1".to_string());
        attributes.insert("auth_key".to_string(), "value2".to_string());

        let mut tasks = HashMap::new();
        tasks.insert(
            "test-task".to_string(),
            Task {
                attributes,
                ..Default::default()
            },
        );

        composer
            .add_workflow(
                "test-workflow".to_string(),
                "0.0.1".to_string(),
                tasks,
                None,
            )
            .unwrap();

        let composer_task = &composer.workflows.borrow()[0].tasks;

        let attributes =
            composer.get_attributes(&composer_task.get("test-task").unwrap().attributes);

        println!("{:#?}", attributes);

        assert!(
            attributes == "[Namespace:\"value1\",AuthKey:\"value2\"]"
                || attributes == "[AuthKey:\"value2\",Namespace:\"value1\"]"
        );
    }

    #[test]
    fn get_kind_test_pass() {
        let composer = Composer::default();

        let kind_name = composer.get_kind("polkadot").unwrap();
        assert_eq!(&kind_name, "Polkadot");

        let kind_name = composer.get_kind("openwhisk").unwrap();
        assert_eq!(&kind_name, "OpenWhisk");
    }

    #[test]
    #[should_panic]
    fn get_kind_test_fail() {
        let composer = Composer::default();
        let kind_name = composer.get_kind("polkadot").unwrap();
        assert_eq!(&kind_name, "polkadot");
    }

    #[test]
    fn generate_wasm_test() {
        let composer = Composer::default();

        let mut input_args: Vec<Input> = Vec::new();
        input_args.push(Input {
            name: "field_1".to_string(),
            input_type: "i32".to_string(),
            ..Default::default()
        });

        let mut depend : Vec<Depend> = Vec::new();
        depend.push(Depend { task_name: "employee_id".to_string(), cur_field: "id".to_string(), prev_field: "ids".to_string() });

        let operation : Operation = task::Operation::Map(String::from("map"));

        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert(
            "api_host".to_string(),
            "https://65.20.70.146:31001".to_string(),
        );
        attributes.insert("auth_key".to_string(), "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string());
        attributes.insert("insecure".to_string(), "true".to_string());
        attributes.insert("namespace".to_string(), "guest".to_string());

        let task = Task::new(
            "OpenWhisk",
            "get_salaries",
            input_args,
            attributes,
            depend,
            operation,
        );

        let mut tasks = HashMap::new();
        tasks.insert("get_salaries".to_string(), task);

        composer
            .add_workflow(
                "test_workflow".to_string(),
                "0.0.1".to_string(),
                tasks,
                None,
            )
            .unwrap();

        let current_path = env::current_dir().unwrap();
        let composer_path = current_path.parent().unwrap();

        composer.generate(composer_path);

        assert!(composer_path
            .join("workflow_wasm/test_workflow-0.0.1.wasm")
            .exists());

        fs::remove_file(composer_path.join("workflow_wasm/test_workflow-0.0.1.wasm")).unwrap();
    }
}
