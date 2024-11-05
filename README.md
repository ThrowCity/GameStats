# GameStats

Rust application to analyze Valorant match data. Whether using `cargo run` to run the app directly or using a pre-built binary, all input files lie within `data/`

## Input

All input files have to be in the `csv` file format, using commas as separators

All percentage values **MUST** be decimals in the 0-1 range when exported to csv.

### Team Input

A file, `data/teams.csv` should be provided, with the following row format:

```csv
teamID,team name,player1,player2,...
```

You may include as many players as necessary per team. There is no need for a header row, however, the addition of one should not break anything so long as it has at least 2 entries.

### Match Input

Individual match results exist as csv files in the `data/matches` directory. Only match csv files should be in this directory. A utility tool, `xlsx2csv.py` has been provided to help turn excel files into multiple individual csv files. Usage:

```sh
python xlsx2csv.py -a -I[filter] raw.xlsx matches/
```

`[filter]` should be replaced with the first character of the pages you want to convert (aim to do 10 pages at a time), and `raw.xlsx` should be replaced with your input file. The individual match csv files should follow this format:

```csv
AGENT,PLAYER,AVG COMBAT SCORE,K,D,A,K/D,ADR,KAST,FK,FD,FDD,HS%,PLANTS,DEFUSES,ECON RATING
<agent>,<player>,<acs>,<k>,<d>,<a>,<kd>,<adr>,<kast>,<fk>,<fd>,<fdd>,<hs>,<plants>,<defuses>,<eco>
<agent>,<player>,<acs>,<k>,<d>,<a>,<kd>,<adr>,<kast>,<fk>,<fd>,<fdd>,<hs>,<plants>,<defuses>,<eco>
<agent>,<player>,<acs>,<k>,<d>,<a>,<kd>,<adr>,<kast>,<fk>,<fd>,<fdd>,<hs>,<plants>,<defuses>,<eco>
<agent>,<player>,<acs>,<k>,<d>,<a>,<kd>,<adr>,<kast>,<fk>,<fd>,<fdd>,<hs>,<plants>,<defuses>,<eco>
<agent>,<player>,<acs>,<k>,<d>,<a>,<kd>,<adr>,<kast>,<fk>,<fd>,<fdd>,<hs>,<plants>,<defuses>,<eco>
<agent>,<player>,<acs>,<k>,<d>,<a>,<kd>,<adr>,<kast>,<fk>,<fd>,<fdd>,<hs>,<plants>,<defuses>,<eco>
<agent>,<player>,<acs>,<k>,<d>,<a>,<kd>,<adr>,<kast>,<fk>,<fd>,<fdd>,<hs>,<plants>,<defuses>,<eco>
<agent>,<player>,<acs>,<k>,<d>,<a>,<kd>,<adr>,<kast>,<fk>,<fd>,<fdd>,<hs>,<plants>,<defuses>,<eco>
<agent>,<player>,<acs>,<k>,<d>,<a>,<kd>,<adr>,<kast>,<fk>,<fd>,<fdd>,<hs>,<plants>,<defuses>,<eco>
<agent>,<player>,<acs>,<k>,<d>,<a>,<kd>,<adr>,<kast>,<fk>,<fd>,<fdd>,<hs>,<plants>,<defuses>,<eco>
,
<winning team>,<losing team>,<group (if applicable)>,<winning score>,-,<losing score>,,<map>
```

If there are less than 10 players, all empty rows **MUST** be kept, just left empty.

## Output

Two output files are generated, `stats.csv` and `agents.csv`, containing the player stats and agent stats respectively. They should be directly imported into either excel or google sheets. Percentages are exported as decimals in the 0-1 range.

## Errors

This program doesn't have advanced error handling. In the event of an error due to invalid input data, it will provide one of the following:
- File origin (csv file in which the error exists)
- Associated Team
- Associated Player

It will also provide a basic description of the error, usually a code which can be explained further. Whenever you get one of these errors, it is advised you check the spelling of teams and players, making sure they are the same across all pages of the sheet.

If an error occurs due to external circumstances, such as incorrect file permissions, there is no guarantee of the level of error logging that will displayed.