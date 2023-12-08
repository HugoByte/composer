attributes = {

    "api_host" : "http://127.0.0.1:7890",

    "auth_key" : "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",

    "insecure" : "true",

    "namespace" : "guest"   

}
 
employee_ids = task(

    kind = "openwhisk",

    action_name = "employee_ids",

    input_arguments = [

        argument(name = "role", input_type = String ),  

    ],

    attributes = attributes,

)
 
getsalaries = task(

    kind = "openwhisk",

    action_name = "getsalaries",

    input_arguments = [

        argument(name = "id", input_type = Int )

    ],

    attributes = attributes,

    operation = Operation.map("salary"),

    depend_on = [

        depend(task_name = "employee_ids", cur_field = "id", prev_field = "ids")

    ]

)
 
employee_salary_workflow = workflows(
    name = "employee_salary_map_cat_test",
    version = "0.0.1",
    tasks = [employee_ids, getsalaries]
)