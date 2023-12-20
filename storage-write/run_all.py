import subprocess

# List of arguments
args = ["blocks", "block_rewards", "transactions", "instructions", "tokens", "token_transfers", "accounts"]

# Loop through each argument and run the command
for arg in args:
    cmd = f"nohup ./subscriber_rabbitmq/subscriber_rabbitmq {arg} > {arg}_out.log 2> {arg}_err.log &"
    subprocess.run(cmd, shell=True)
