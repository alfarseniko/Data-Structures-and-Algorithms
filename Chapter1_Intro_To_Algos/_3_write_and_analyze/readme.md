Algorithm Swap (a, b)
        Begin
            temp = a;
            a = b;
            b = temp;
        End

    Analyze an Algorithm
        1. Time
            Algos should be faster
            We get a time function
            We assume that each line takes one unit of time. In this case
                        f(n) = 1 + 1 + 1
                        f(n) = 3
                    In order of one O(1) which means that its constant
                    2. Space
            The memory required to run the algorithm
                Currently there are three variables that need to be stored
                        temp → 1 unit of space
                        a → 1 unit of space
                        b → 1 unit of space
                            s(n) = 3
                        Can also be represented as s(n) = 3 words
                        The type and actual number of bits in a word is dependent on the computer
                In order of one O(1) which means that its constant
        3. Network
            The amount of data that needs to be sent through
            the internet, and the time and speed at which it goes through
        4. Power Consumption
            Electriccal power used, one should know the electrical properties
            of the computing device
        5. CPU Registers
           Optimizing the CPU Registers