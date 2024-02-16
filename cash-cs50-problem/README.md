This project is just for me to recreate the CS50 week 1 problem of cash.

The problem is as follows you should have the following functions,I'm too lazy to explain:

`int get_cents(void)
{
    int cents;
    do
    {
        printf("Change owed: ");
        scanf("%i", &cents);
    }
    while (cents < 0);

    return cents;
}`

`quaters is cents/25`
`dimes is cents/10`
`nickels is cents/5`
`pennies are the same as cents ie cents/1`