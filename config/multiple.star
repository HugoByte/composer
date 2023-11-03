attributes = {
    "api_host" : "https://65.20.70.146:31001",
    "auth_token" : "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
    "insecure" : "true",
    "namespace" : "guest"   
}

employee_id = task(
    kind = "openwhisk",
    action_name = "employee_ids",
    input_args = {
        "role" : "String"
    },
    attributes = attributes,
    depend_on = {}
)

getsalaries = task(
    kind = "openwhisk",
    action_name = "getsalaries",
    input_args = {
        "id" : "i32"
    },
    attributes = attributes,
    operation = "map",
    depend_on = {
        
        "employee_ids" : {
            "id" : "id"
        }
    }
)

getaddress = task(
    kind = "openwhisk",
    action_name = "getaddress",
    input_args = {
        "id" : "i32"
    },
    attributes = attributes,
    operation = "map",
    depend_on = {
        "employee_ids" : {
            "id" : "id"
        }
    },
    
)

salary = task(
    kind = "openwhisk",
    action_name = "salary",
    input_args = {
        "details" : "HashMap<i32,(i32,String)>"
    },
    attributes = attributes,
    depend_on = {
        "getsalaries" : {
            "details" : "result"
        },
        "getaddress" : {
            "details" : "result"
        }
    }
)

stakingpayout = task(
    kind = "polkadot",
    action_name = "stakingpayout",
    input_args = {
        "url" : "String",
        "owner_key" : "String",
        "address" : "String",
        "era" : "u32"
    },
    attributes = {
        "chain" : "westend",
        "operation" : "stakingpayout"
    },
    depend_on = { }
)

workflow_employee = workflows(
    name = "workflow",
    version = "0.0.1",
    task_name = [employee_id, getsalaries, getaddress, salary]
)

workflow_polkadot = workflows(
    name = "workflow",
    version = "0.0.1",
    task_name = [stakingpayout]
)
