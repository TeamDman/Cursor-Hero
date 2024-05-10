# Currently working

- inspector
  - marking elements
  - scratch pad
- virtual calculator
  - represents state in visible text

# Limitations

- z index problem
- primitive element rediscovery
- calculator has hidden state
- calculator doesn't evaluate
- calculator missing buttons

# Future work

- more automatic element scraping
  - hover color, text color, font family, font size
  - [![](https://private-user-images.githubusercontent.com/774615/289379016-5fa6d008-4042-40ea-b3e6-f97ef4dd83db.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MTUwMzQ5NjEsIm5iZiI6MTcxNTAzNDY2MSwicGF0aCI6Ii83NzQ2MTUvMjg5Mzc5MDE2LTVmYTZkMDA4LTQwNDItNDBlYS1iM2U2LWY5N2VmNGRkODNkYi5wbmc_WC1BbXotQWxnb3JpdGhtPUFXUzQtSE1BQy1TSEEyNTYmWC1BbXotQ3JlZGVudGlhbD1BS0lBVkNPRFlMU0E1M1BRSzRaQSUyRjIwMjQwNTA2JTJGdXMtZWFzdC0xJTJGczMlMkZhd3M0X3JlcXVlc3QmWC1BbXotRGF0ZT0yMDI0MDUwNlQyMjMxMDFaJlgtQW16LUV4cGlyZXM9MzAwJlgtQW16LVNpZ25hdHVyZT1lYjg1ODc3NDNmMmY5Zjc5NDdkODEyN2YxYWQ4YmMyMjMzMmJlZDAwZTg0NzA2M2NlNDVlMGQ0ZGE1YmI1MTViJlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCZhY3Rvcl9pZD0wJmtleV9pZD0wJnJlcG9faWQ9MCJ9.DtK60ywubZjxPIKh6PgH1ebH4LnIO36c2mBMNGIPNrA)](https://github.com/OpenAdaptAI/OpenAdapt)
  - https://github.com/SysCV/sam-hq
- evaluation impl by 
- random walk for state transition test cases
- path finding
  - given a desired state, find the shortest path to get there
  - goal: expression="1000000+" value="1"
  - path1: 1,0,0,0,0,0,0,+,1
  - path2: 6,10^x,=,+,1 
    - equals is necessary
