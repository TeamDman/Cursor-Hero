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

# Future work

- AI to pick significant elements in screenshot
- scrape: hover color, text color, font family, font size
- random walk for state transition test cases
- path finding
  - given a desired state, find the shortest path to get there
  - goal: expression="1000000+" value="1"
  - path1: 1,0,0,0,0,0,0,+,1
  - path2: 6,10^x,=,+,1 
    - equals is necessary
