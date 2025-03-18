MODEL
    NAMESPACE https://decision-toolkit.org/2_0001/
    NAME 2_0001
    ID _2_0001
    VERSION https://www.omg.org/spec/DMN/20191111/MODEL/
    DESCRIPTION
        Compliance level 2: Test 0001

        The decision named **Greeting Message** has a label defined in diagram definition.
        In the diagram this decision is depicted as **GREETING MESSAGE**.
        The output variable name remains **Greeting Message**.

definitions:
  - decision:
      name: "Greeting Message"
      id: _75b3add2-4d36-4a19-a76c-268b49b2f436
      description: |
        This decision prepares a greeting message.
        "Hello" is prepended to the value of the input variable named 'Full Name'.
      question: |
        What is the greeting suitable for our customer?
      allowedAnswers: |
        The proper greeting is in the format: Hello {customer's full name}
      variable:
        name: "Greeting Message"
        id: _3215b422-b937-4360-9d1c-4c677cae5dfd
        typeRef: string
        label: "GREETING MESSAGE"
      informationRequirement:
        - id: _70c3f69a-63f3-4197-96ce-b206c8bd2a6b
          requiredInput:
            href: "#_cba86e4d-e91c-46a2-9176-e9adf88e15db"
      literalExpression:
        id: _5baa6245-f6fc-4685-8973-fa873817e2c1
        text: |
          "Hello " + Full Name
  - inputData:
      name: "Full Name"
      id: _cba86e4d-e91c-46a2-9176-e9adf88e15db
      description: |
        Full name of the customer provided by calling service.
      variable:
        name: "Full Name"
        id: _4bc2161f-2f3b-4260-b454-0a01aed0e46b
        label: "Customer's name"
        typeRef: string
        description: |
          Full name of the person that will be sent greetings from this decision model.

diagrams:
  - diagram:
      name: Decision Requirement Diagram
      id: _d3a3312e-5924-4f7b-ac0e-232ef9203ff6
      resolution: 300
      size: 190.0 240.0
      shapes:
        - dmnElementRef: _75b3add2-4d36-4a19-a76c-268b49b2f436
          id: _ebf33cfc-0ee3-4708-af8b-91c52237b7d6
          bounds: 20.0 20.0 150.0 60.0
          label:
            text: GREETING MESSAGE
            sharedStyle: style1
          sharedStyle: style1
        - id: _48ea7a1d-2575-4cb7-8b63-8baa4cb3b371
          dmnElementRef: _cba86e4d-e91c-46a2-9176-e9adf88e15db
          bounds: 20.0 160.0 150.0 60.0
          sharedStyle: style2
      edges:
        - dmnElementRef: _70c3f69a-63f3-4197-96ce-b206c8bd2a6b
          id: _e9a73517-0ba2-4b31-b308-82279ae21591
          waypoints:
            - point: 95.0 160.0
            - point: 95.0 80.0
      styles:
        - id: style1
          font: bold underline italic strikethrough 18 Arial, Helvetica, sans-serif
          labelVerticalAlignment: start
          fillColor: 10 255 255
          strokeColor: 255 0 0
          fontColor: 0 200 0
        - id: style2
          font: bold underline 12 Arial, "Fira Sans", sans-serif
          strokeColor: 255 0 0
