# time-converter

Transformer between DateTime\<Tz> and timestamp_nanos

## Run

```shell
./time-converter timestamp 1695627566000000000
./time-converter datetime 2023-09-25T07:39:26+00:00
./time-converter now
```

## Alias

```text
alias ts="time-converter timestamp"     # ts 1696558217281930000
alias dt="time-converter datetime"      # dt 2023-10-06T11:10:17.281930+09:00
alias now="time-converter now"          # now
```
