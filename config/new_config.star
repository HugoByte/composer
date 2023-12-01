
typ(
    name = "struct1",
    fields = {
        "field1" : string(),
        "field2" : bool(),
        "field3" : int(16)
    }
)

typ(
    name = "struct2",
    fields = {
        "field1" : map(int(8), string()),
        "field2" : list(string()),
    }
)

typ(
    name = "detailtype",
    fields = {
        "field1" : int(32),
        "field2" : string(),
    }
)

attributes = {
    "api_host" : "https://65.20.70.146:31001",
    "auth_key" : "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
    "insecure" : "true",
    "namespace" : "guest"   
}

employee_id = task(
    kind = "openwhisk",
    action_name = "employee_ids",
    input_args = [
        input_args(name = "role", input_type = string() ),  
    ],
    attributes = attributes,
    depend_on = []
)

getsalaries = task(
    kind = "openwhisk",
    action_name = "getsalaries",
    input_args = [
        input_args(name = "id", input_type = int(32), default_value = "23" )
    ],
    attributes = attributes,
    operation = operation(operation = "map", field = "Salary"),
    depend_on = [
        depend(task_name = "employee_ids", cur_field = "id", prev_field = "ids")
    ]
)

getaddress = task(
    kind = "openwhisk",
    action_name = "getaddress",
    input_args = [
        input_args(name = "id", input_type = int(32), default_value = "1")
    ],
    attributes = attributes,
    operation = operation(operation = "map", field = "Address"),
    depend_on = [
        depend(task_name = "employee_ids", cur_field = "id", prev_field = "ids")
    ],
    
)

salary = task(
    kind = "openwhisk",
    action_name = "salary",
    input_args = [
        input_args(name = "details", input_type = map(int(32), "(i32, String)"))
    ],
    attributes = attributes,
    operation = operation("concat"),
    depend_on = [
        depend(task_name = "getsalaries", cur_field = "details", prev_field = "result"),
        depend(task_name = "getaddress", cur_field = "details", prev_field = "result")
    ]
)


employee_salary_workflow = workflows(
    name = "employee_salary",
    version = "0.0.1",
    tasks = [employee_id, getsalaries, getaddress, salary],
    custom_types = [Struct("struct1"), Struct("struct2")]
)