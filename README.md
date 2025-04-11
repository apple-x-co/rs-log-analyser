# rs-log-analyser

## Usage


```bash
rs-log-analyser --input access_log --pattern '22/Feb/2025:\d\d'
```

### Output format

```text
<MATCHED TEXT>\t<COUNT>
```

### Output sample

```text
22/Feb/2025:14	10167
22/Feb/2025:06	2769
22/Feb/2025:07	5617
22/Feb/2025:19	11452
22/Feb/2025:23	10891
22/Feb/2025:13	8597
22/Feb/2025:05	1969
22/Feb/2025:12	8993
22/Feb/2025:21	11195
22/Feb/2025:09	8042
22/Feb/2025:16	9562
22/Feb/2025:04	346
22/Feb/2025:10	7730
22/Feb/2025:11	9216
22/Feb/2025:22	11182
22/Feb/2025:08	8643
22/Feb/2025:18	11245
22/Feb/2025:20	12195
22/Feb/2025:15	9588
22/Feb/2025:17	7674
```