attributes = {
    "api_host": "http://127.0.0.1:8080",
    "auth_key": "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",
    "insecure": "true",
    "namespace": "guest",
}

cartype = task(
    kind = "openwhisk",
    action_name = "cartype",
    input_args = [
        input_args(
            name="car_type",
            input_type= string()
        ),
    ],
    attributes = attributes
)

modelavail = task(
    kind = "openwhisk",
    action_name = "modelavail",
    input_args = [
        input_args(
            name="car_company_list",
            input_type = hashmap(string(), list(string()))
        ),
        input_args(
            name="company_name",
            input_type=string()
        )
    ],
    attributes = attributes,
    depend_on = [
        depend(task_name = "cartype", cur_field = "car_company_list", prev_field = "car_company_list")
    ]
)

modelprice = task(
    kind = "openwhisk",
    action_name = "modelsprice",
    input_args = [
        input_args(
            name="models",
            input_type=list(string())
        ),
    ],
    attributes = attributes,
    depend_on = [
        depend(task_name = "modelavail", cur_field = "models", prev_field = "models")
    ]
)

purchase = task(
    kind = "openwhisk",
    action_name = "purchase",
    input_args = [
        input_args(
            name="model_price_list",
            input_type = hashmap(string(), int(32))
        ),
        input_args(
            name="model_name",
            input_type=string()
        ),
        input_args(
            name="price",
            input_type=int(32)
        ),
    ],
    attributes = attributes,
    depend_on = [
        depend(task_name = "modelsprice", cur_field = "model_price_list", prev_field = "model_price_list")
    ]
)

workflows(
    name = "car_market_place",
    version = "0.0.1",
    tasks = [cartype, modelavail, modelprice, purchase],
)