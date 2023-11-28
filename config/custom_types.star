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
        "field1" : hashmap(int(8), string()),
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
    depend_on = {}
)

getsalaries = task(
    kind = "openwhisk",
    action_name = "getsalaries",
    input_args = [
        input_args(name = "id", input_type = int(32), default_value = "23" )
    ],
    attributes = attributes,
    operation = operation(operation = "map", field = "Salary"),
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
        input_args(name = "id", input_type = int(32), default_value = "1")
    ],
    attributes = attributes,
    operation = operation(operation = "map", field = "Address"),
    depend_on = {
        "employee_ids" : {
            "id" : "ids"
        }
    },
    
)

salary = task(
    kind = "openwhisk",
    action_name = "salary",
    input_args = [
        input_args(name = "details", input_type = hashmap(int(32), "(i32, String)"))
    ],
    attributes = attributes,
    operation = operation("concat"),
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
    input_args = [
        input_args(name = "url", input_type = "String", default_value = "wss://rpc.polkadot.io"),
        input_args(name = "owner_key", input_type = "String", default_value = "caution juice atom organ advance problem want pledge someone senior holiday very"),
        input_args(name = "address", input_type = "String", default_value = "15oF4uVJwmo4TdGW7VfQxNLavjCXviqxT9S1MgbjMNHr6Sp5"),
        input_args(name = "era", input_type = "String", default_value = "1"),
    ],
    attributes = {
        "chain" : "westend",
        "operation" : "stakingpayout"
    },
    depend_on = { }
)

employee_salary_workflow = workflows(
    name = "employee_salary",
    version = "0.0.1",
    tasks = [employee_id, getsalaries, getaddress, salary],
    custom_types = [Struct("struct1"), Struct("struct2")]
)

# workflow_polkadot_workflow = workflows(
#     name = "polkadot_payout",
#     version = "0.0.1",
#     tasks = [stakingpayout],
#     custom_types = []
# )