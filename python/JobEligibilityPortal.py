print("Welcome to the Python Developer Job Portal!")
print("-" * 45)

python = input("Do you know Python? (yes/no): ").strip().lower()
experience = int(input("How many years of experience or projects do you have? "))
degree = input("Do you have a CS degree or completed a Bootcamp? (yes/no): ").strip().lower()

if python == "yes" and (experience >= 2 or degree == "yes"):
    print("Congratulations! You have been accepted to the next interview stage.")
else:
    print("Sorry, your current qualifications do not match the job requirements.")