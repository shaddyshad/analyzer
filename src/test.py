class ClassToken(Super):
    """ 
        Token representing a class 
    """
    def __init__(self):
        """ 
            Initialize class attributes 
        """
        self.name = ""
        self.parent = None 
        self.attributes = []
        self.constructor = None 
        self.num_of_methods = 0 
        self.docstring = ""

# a comment
a = ClassToken()
b = "shad"

if a == b:
    print("A is greater")

for i in range(10):
    if i > 5:
        print("Greater")