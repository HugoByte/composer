attributes = {
    "api_host" : "http://127.0.0.1:8080",
    "auth_key" : "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
    "insecure" : "true",
    "namespace" : "guest"   
}

employee_ids = task(
    kind = "openwhisk",
    action_name = "employee_ids",
    input_args = [
        input_args(name = "role", input_type = string() ),  
    ],
    attributes = attributes,
)

getsalaries = task(
    kind = "openwhisk",
    action_name = "getsalaries",
    input_args = [
        input_args(name = "id", input_type = int(32) )
    ],
    attributes = attributes,
    operation = Operation.map("salary"),
    depend_on = [
        depend(task_name = "employee_ids", cur_field = "id", prev_field = "ids")
    ]
)

getaddress = task(
    kind = "openwhisk",
    action_name = "getaddress",
    input_args = [
        input_args(name = "id", input_type = int(32))
    ],
    attributes = attributes,
    operation = Operation.map("address"),
    depend_on = [
        depend(task_name = "employee_ids", cur_field = "id", prev_field = "ids")
    ]
)

salary = task(
    kind = "openwhisk",
    action_name = "salary",
    input_args = [
        input_args(name = "details", input_type = hashmap(int(32), "(i32, String)"))
    ],
    attributes = attributes,
    operation = Operation.concat(),
     depend_on = [
        depend(task_name = "getsalaries", cur_field = "details", prev_field = "result"),
        depend(task_name = "getaddress", cur_field = "details", prev_field = "result")
    ]
)

employee_salary_workflow = workflows(
    name = "employee_salary_map_cat_test",
    version = "0.0.1",
    tasks = [employee_ids, getsalaries, getaddress, salary]
)