#!/usr/bin/env python3
import time
from datetime import datetime

import modal


question = "Quadcast HyperX condenser mic, connects via usb-c to your computer for crystal clear audio"

#### 1. function
# $ modal deploy -m price_fn

pricer = modal.Function.from_name("pricer-service", "price")
reply = pricer.remote(question)

print(reply)


#### 2. class
# $ modal deploy -m pricer_cls

Pricer = modal.Cls.from_name("pricer-service", "Pricer")
pricer = Pricer()
reply = pricer.price.remote(question)

print(reply)


#### 3. keep warm
while True:
    reply = pricer.wake_up.remote()
    print(f"{datetime.now()}: {reply}")
    time.sleep(30)


#### 4. list modal apps
# $ modal app list
