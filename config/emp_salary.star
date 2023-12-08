attributes = {
    "api_host" : "https://65.20.70.146:31001",
    "auth_token" : "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
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
        input_args(name = "id", input_type = int(32) )
    ],
    attributes = attributes,
    operation = operation("map", "salary"),
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
    operation = operation("map", "address"),
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
    operation = operation("concat"),
     depend_on = [
        depend(task_name = "getsalaries", cur_field = "details", prev_field = "result"),
        depend(task_name = "getaddress", cur_field = "details", prev_field = "result")
    ]

)

stakingpayout = task(
    kind = "polkadot",
    action_name = "stakingpayout",
    input_args = [
        input_args(name = "url", input_type = "String"),
        input_args(name = "owner_key", input_type = "String"),
        input_args(name = "address", input_type = "String"),
        input_args(name = "era", input_type = "String"),
    ],
    attributes = {
        "chain" : "westend",
        "operation" : "stakingpayout"
    },
    depend_on = []
)

employee_salary_workflow = workflows(
    name = "employee_salary",
    version = "0.0.1",
    tasks = [employee_id, getsalaries, getaddress, salary]
)

workflow_polkadot_workflow = workflows(
    name = "polkadot_payout",
    version = "0.0.1",
    tasks = [stakingpayout]
)