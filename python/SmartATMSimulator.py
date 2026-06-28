# import random
# number = random.randint(4000, 5000)

pin = 1234
balance = 5000

pin_input = int(input("Please enter your 4-digit PIN: ").strip()) 

if pin_input == pin:

    case = input("would you like to withdraw or access your balance?").strip().lower()

    if case == "access balance":
        print(f"Your current balance is: ${balance}")

    elif case == "withdraw":

        amount = float(input("Enter the amount you want to withdraw: ").strip())

        if amount > 0:
           
            if amount <= balance:

                balance -= amount

                print(f"Withdrawal successful ${amount}! Your new balance is: ${balance}")
                
            else:
                print("Sorry, your balance is insufficient.")
        else:        
            print("Invalid amount.")      
              
    else: 
         print("Invalid option. Please choose 'withdraw' or 'access balance'.")
        

else:
    print("Incorrect PIN. Please try again.")