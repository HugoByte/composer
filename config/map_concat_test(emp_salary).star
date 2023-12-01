attributes = {
    "api_host" : "http://127.0.0.1:8080",
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
)

getsalaries = task(
    kind = "openwhisk",
    action_name = "getsalaries",
    input_args = [
        input_args(name = "id", input_type = int(32) )
    ],
    attributes = attributes,
    operation = Operation.map("salary"),
    depend_on = {
        "employee_ids" : {
            "id" : "ids"
        }
    }
)

getaddress = task(
    kind = "openwhisk",
    action_name = "getaddress",
    input_args = [
        input_args(name = "id", input_type = int(32))
    ],
    attributes = attributes,
    operation = Operation.map("address"),
    depend_on = {
        "employee_ids" : {
            "id" : "ids"
        }
    }
)

salary = task(
    kind = "openwhisk",
    action_name = "salary",
    input_args = [
        input_args(name = "details", input_type = hashmap(int(32), "(i32, String)"))
    ],
    attributes = attributes,
    operation = Operation.concat(),
     depend_on = {
        "getsalaries" : {
            "details" : "result"
        },
        "getaddress" : {
            "details" : "result"
        },
     }
)

employee_salary_workflow = workflows(
    name = "employee_salary_map_cat_test",
    version = "0.0.1",
    tasks = [employee_id, getsalaries, getaddress, salary]
)