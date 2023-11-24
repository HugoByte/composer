stakingpayout = task(
    kind = "Polkadot",
    action_name = "stakingpayout",
    input_args = [
        input_args(
            name="url",
            input_type= string()
        ),
        input_args(
            name="owner_key",
            input_type= string()
        ),
        input_args(
            name="address",
            input_type= string()
        ),
        input_args(
            name="era",
            input_type= string()
        ),
    ],
    depend_on = {},
)

push_notification = task(
    kind = "Openwhisk",
    action_name = "push_notification",
    input_args = [
        input_args(
            name="token",
            input_type= string() 
        ),
        input_args(
            name="message",
            input_type="Value"
        ),
        input_args(
            name="result",
            input_type="Option<H256>"
        ),

    ],
    attributes = attributes,
    depend_on = {
        "stakingpayout": {
            "result": "result" 
        },
    },
)

workflows(
    name = "payout_notification",
    version = "0.0.1",
    tasks = [stakingpayout, push_notification],
)