enum token_type{ 
	TAB,
	LINE_END,
	STAR,
	NUMBER_LITTERAL,
	FLOAT_LITTERAL,
	ADD,
	SUBTRACT,
	MULTIPLY,
	DIVIDE,
};
union token_data{
	int number_litteral;
	float float_litteral;
};
struct token{
	enum token_type type;
	union token_data data;
	unsigned int line_number;
};
struct token_array{
	struct token *ptr;
	unsigned int len;
};
struct token_array lexical_analysis(char *text){
}
