# Elgato-rs

## Notes

light-left = 192.168.1.24

## API: Elgato Key Lights

Turn light on:

```curl -v "http://192.168.1.24:9123/elgato/lights" -XPUT -H 'Content-Type: application/json' -d '{"numberOfLights":1,"lights":[{"on":1}]}'```

Turn light off:

```curl -v "http://192.168.1.24:9123/elgato/lights" -XPUT -H 'Content-Type: application/json' -d '{"numberOfLights":1,"lights":[{"on":0}]}'```
