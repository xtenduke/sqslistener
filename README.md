# sqslistener

Run scripts from AWS SQS messages
Listens to a SQS queue, and runs a configured 'script' / 'command'

### Uses:
- Deploying software / container images to a server without exposing SSH
- Scheduling server updates / maintenance

### Configuration
A sample configuration file is provided [here](./example.sqslistener.toml)<br>


### Depends on
- AWS credentials in your env. i.e. from AWS Configure.
- The role need only `sqs:ReceiveMessage` and `sqs:DeleteMessage`

```
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "",
            "Effect": "Allow",
            "Action": [
                "sqs:ReceiveMessage",
                "sqs:DeleteMessage"
            ],
            "Resource": "<your-queue-arn>"
        }
    ]
}
```

## Installation
Install from the provided deb or rpm packages, or install manually.


### Manual installation with install.sh
From the directory you downloaded the release
```
unzip sqslistener-linux-x86.zip
cd sqslistener-linux-x86
./install.sh
```

### Adding the systemd service
- This is not needed if you installed with the script
- Copy this example systemd unit to `/etc/systemd/system/sqslistener.service`
- Replace the `USER_PLACEHOLDER` value in the unit with the username you want the service to be executed by
- `User=USER_PLACEHOLDER` becomes `User=milesobrien`
- Start and enable the service
- `sudo systemctl start sqslistener.service && sudo systemctl enable sqslistener.service`
sqslistener.service
```
[Unit]
Description=sqs listener

[Service]
Type=simple
Environment="RUST_LOG=debug,info"
ExecStart=/usr/bin/sqslistener
User=USER_PLACEHOLDER

[Install]
WantedBy=multi-user.target
```

### Logging
View journal logs with `journalctl -f -u sqslistener`<br>
Proper persistent log is coming


# TODO:
- Better logging
- Unit testing
- Hanged process / max execution handling 
    - currently a hanged child process could result in a single task being executed multiple times due to SQS message visibility timeout
- Packaging