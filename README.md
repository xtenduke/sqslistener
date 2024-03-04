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

## Installation (systemd)

#### installation.sh
From the directory you downloaded the release
```
unzip sqslistener-linux-x86.zip
cd sqslistener-linux-x86
./install.sh
```

### Manual install (systemd)
From the directory you downloaded the release.<br>
##### Installing the binary:
```
cd sqslistener-linux-x86
chmod +x sqslistener
sudo cp sqslistener /usr/bin
```

##### Adding the systemd service
- Open the `installation/sqslistener.service` file with your favorite text editor
- Replace the `USER_PLACEHOLDER` value with the username you want the service to be executed by
- `User=USER_PLACEHOLDER` becomes `User=milesobrien`

Install and start the service
```
sudo cp installation/sqslistener.service /etc/systemd/system/sqslistener.service
sudo systemctl start sqslistener.service
sudo systemctl enable sqslistener.service
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