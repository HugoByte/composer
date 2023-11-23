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
            input_type="String"
        ),
    ],
    attributes = attributes,
    depend_on = {},
)

modelavail = task(
    kind = "openwhisk",
    action_name = "modelavail",
    input_args = [
        input_args(
            name="car_company_list",
            input_type="HashMap<String,Vec<String>>"
        ),
        input_args(
            name="company_name",
            input_type="String"
        )
    ],
    attributes = attributes,
    depend_on = {
        "cartype": {
            "car_company_list": "car_company_list",
        },
    },
)

modelprice = task(
    kind = "openwhisk",
    action_name = "modelsprice",
    input_args = [
        input_args(
            name="models",
            input_type="Vec<String>"
        ),
    ],
    attributes = attributes,
    depend_on = {
        "modelavail": {
            "models": "models",
        },
    },
)

purchase = task(
    kind = "openwhisk",
    action_name = "purchase",
    input_args = [
        input_args(
            name="model_price_list",
            input_type="HashMap<String,i32>"
        ),
        input_args(
            name="model_name",
            input_type="String"
        ),
        input_args(
            name="price",
            input_type="i32"
        ),
    ],
    attributes = attributes,
    depend_on = {
        "modelsprice": {
            "model_price_list": "model_price_list",
        },
    },
)

workflows(
    name = "car_market_place",
    version = "0.0.1",
    tasks = [cartype, modelavail, modelprice, purchase],
)