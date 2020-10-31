class ClassToken:
    """ Token representing a class """
    def __init__(self):
        """ Initialize class attributes """
        self.name = ""
        self.parent = None 
        self.attributes = []
        self.contructor = None 
        self.num_of_methods = 0 
        self.docstring = ""
a = ClassToken()