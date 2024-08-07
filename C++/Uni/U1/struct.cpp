// Structures are used for group together elements with dissimilar types.

struct book
{
	chartitle[25];
	charauthor[25];
	int pages;
	float price;
};
struct book book1, book2, book3;
book1.pages = 550;
book2.pages = 225.75;

// Size of Union is equal to size of its largest member of the element.
union result
{
	int marks;
	char grade;
	float precent;
};
