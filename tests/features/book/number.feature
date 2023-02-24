Feature: Bug Hunting in Factorial Website

  Scenario Outline: Verify the factorial of a number
    Given I enter <number> in the input box
    When I click submit
    Then I check that the answer is <answer>

  Examples: 
    | number | answer |
    | 2      | 2 |
    | 3      | 6 |
    | 4      | 24 |  
    | 5      | 120 |  