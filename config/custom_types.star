type1= typ(
    name = "struct1",
    fields = {
        "field1" : string(),
        "field2" : bool(),
        "field3" : int(16)
    }
)

type2= typ(
    name = "struct2",
    fields = {
        "field1" : map(int(8), string()),
        "field2" : list(string()),
    }
)

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
        ip_args(name = "input_field_1", input_type = type1 ),
        ip_args(name = "input_field_1", input_type = type2 ),  
    ],
    attributes = attributes,
    depend_on = {}
)

getsalaries = task(
    kind = "openwhisk",
    action_name = "getsalaries",
    input_args = [
        ip_args(name = "id", input_type = type1, default_value = "23" )
    ],
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
    input_args = [
        ip_args(name = "id", input_type = "i32", default_value = "1")
    ],
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
    input_args = [
        ip_args(name = "details", input_type = "HashMap<i32,(i32,String)>", default_value = "{1:(42,Hello)}")
    ],
    attributes = attributes,
    operation = "concat",
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
        ip_args(name = "url", input_type = "String", default_value = "wss://rpc.polkadot.io"),
        ip_args(name = "owner_key", input_type = "String", default_value = "caution juice atom organ advance problem want pledge someone senior holiday very"),
        ip_args(name = "address", input_type = "String", default_value = "15oF4uVJwmo4TdGW7VfQxNLavjCXviqxT9S1MgbjMNHr6Sp5"),
        ip_args(name = "era", input_type = "String", default_value = "1"),
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
    custom_types = [type1, type2]
)

workflow_polkadot_workflow = workflows(
    name = "polkadot_payout",
    version = "0.0.1",
    tasks = [stakingpayout],
    custom_types = []
)