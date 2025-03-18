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

DECISION
    NAME Greeting Message
    ID _75b3add2-4d36-4a19-a76c-268b49b2f436
    DESCRIPTION
        This decision prepares a greeting message.
        "Hello" is prepended to the value of the input variable named 'Full Name'.
    QUESTION
        What is the greeting suitable for our customer?
    ANSWERS
        The proper greeting is in the format: Hello {customer's full name}
    VARIABLE
        NAME Greeting Message
        ID _3215b422-b937-4360-9d1c-4c677cae5dfd
        TYPE-REF string
        LABEL `GREETING MESSAGE`
    INFORMATION-REQUIREMENT
        ID _70c3f69a-63f3-4197-96ce-b206c8bd2a6b
        INPUT #_cba86e4d-e91c-46a2-9176-e9adf88e15db
    LITERAL_EXPRESSION
        ID _5baa6245-f6fc-4685-8973-fa873817e2c1
        TEXT "Hello " + Full Name

INPUT-DATA
    NAME Full Name
    ID _cba86e4d-e91c-46a2-9176-e9adf88e15db
    DESCRIPTION Full name of the customer provided by calling service.
    VARIABLE
        NAME "Full Name"
        ID _4bc2161f-2f3b-4260-b454-0a01aed0e46b
        LABEL Customer's name
        TYPE-REF string
    DESCRIPTION
        Full name of the person that will be sent greetings from this decision model.

DIAGRAM
    NAME Decision Requirement Diagram
    ID _d3a3312e-5924-4f7b-ac0e-232ef9203ff6
    RESOLUTION 300
    SIZE 190.0 240.0
    SHAPE
        DMN-ELEMENT-REF _75b3add2-4d36-4a19-a76c-268b49b2f436
        ID _ebf33cfc-0ee3-4708-af8b-91c52237b7d6
        BOUNDS 20.0 20.0 150.0 60.0
        LABEL
            TEXT `GREETING MESSAGE`
            SHARED-STYLE style1
        SHARED-STYLE style1
    SHAPE
        DMN-ELEMENT-REF _cba86e4d-e91c-46a2-9176-e9adf88e15db
        ID _48ea7a1d-2575-4cb7-8b63-8baa4cb3b371
        BOUNDS 20.0 160.0 150.0 60.0
        SHARED-STYLE style2
    EDGE
        ID _e9a73517-0ba2-4b31-b308-82279ae21591
        DMN-ELEMENT-REF: _70c3f69a-63f3-4197-96ce-b206c8bd2a6b
        WAYPOINTS 95.0 160.0, 95.0 80.0
    STYLE
        ID style1
        FONT bold underline italic strikethrough 18 Arial, Helvetica, sans-serif
        LABEL-VERTICAL-ALIGN start
        FILL-COLOR 10 255 255
        STROKE-COLOR 255 0 0
        FONT-COLOR 0 200 0
    STYLE
        ID style2
        FONT bold underline 12 Arial, "Fira Sans", sans-serif
        STROKE-COLOR 255 0 0
