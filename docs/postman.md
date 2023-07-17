# Postman Usage

1. enter this as the url
```
grpc://localhost:50051
```

2. upload proto file

3. pick an RPC method to trigger
set the message based on your proto request message

ie. counter.proto
```
message IncrementRequest {
    int32 value = 1;
}
```

we just need to pass
```
{
    "value": 10
}
```

