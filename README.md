* Restructure client usage
```
let config = cvp::Config::from_env();
let config = cvp::Config::from_file("filename");
let config = cvp::Config::new(hostname,port,token);
let client = cvp::Client::new(&config);
let resp = client.get_tags().send().await?; // does this mean we have a send trait?
let tags = tags.tags().unwrap();
// use this for partial_eq_filter()
let resp = client.get_tags().filters(Filter::builder().key("k").build());

let tag = resp.tags().unwrap().first().unwrap().tag().unwrap();
```

Config file format:
Do we need to account for multiple hosts?
```
[cloudvision]
hostname=
port=
token=
```

env vars:
CLOUDVISION_HOSTNAME
CLOUDVISION_PORT
CLOUDVISION_TOKEN
