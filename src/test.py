from .token import ClassToken

class Tokenizer:
    def __init__(self):
        self.tokens = []  # collected tokens so far 
        self.blocks = []    # total blocks
        self.current_block = None   # current block 
        self.identation_level = 0       # track the current identation level 


    """ 
        Tokenizes input data into a tree 
    """
    def tokenize_raw(self, raw_text):
        """ Tokenize raw text """
        pass 

    def tokenize_file(self, filepath):
        """ Tokenize the input from a file """
        pass 

    def _tokenize(self, text):
        """ Tokenize input source """

        # split the text by newline 
        lines = text.split('\n')

        # read each line parsing 
        for i in range(len(lines)):
            if line != "":
                #find the token 
                token = self.find_token(line, i)

    def find_token(self, line, line_number):
        """ Determine the token in a given line """
        first_char_index = self.index_of_first_char(line)

        if first_char_index > 0:
            # probably an identation level 

    def index_of_first_char(self, line):
        """ Find the index of the first character """
        split = line.split('')
        index = 0 

        for i in split:
            if str(i).isspace():
                index += 1 
            else:
                break 

        return index 

