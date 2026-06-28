print("Welcome, dear customer!")

amount = float(input("Please enter the amount: "))

if amount <= 0:
    print("The amount cannot be zero or negative!")

else:

    if amount >= 500:
        discount = amount * 0.2

    elif amount >= 100:
        discount = amount * 0.1
    else:
        discount = 0

    print(f"Your available discount is: {round(discount, 2)}")
    print(f"Your total amount after discount is: {round(amount - discount, 2)}")