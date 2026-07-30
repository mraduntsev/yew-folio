grades_main: list[tuple[int, str]] = [
    (4, "Physics"),
    (4, "Foreign Language"),
    (4, "Probability Theory"),
    (4, "Software Engineering Fundamentals"),
    (4, "Database Systems"),
    (4, "Optimization Methods"),
    (5, "Numerical Methods"),
    (5, "Philosophy"),
    (4, "History"),
    (4, "Economics"),
    (5, "Calculus"),
    (4, "Algebra"),
    (5, "Geometry"),
    (4, "Discrete Mathematics"),
    (5, "Differential Equations"),
    (4, "Russian Language and Speech Culture"),
    (5, "Theoretical Mechanics"),
    (5, "Game Theory and Operations Research"),
    (5, "Linear Programming"),
    (4, "Computer Science"),
    (4, "Computer Graphics"),
    (4, "Information Theory"),
    (4, "Mathematical Modeling in Economics"),
    (5, "Equations of Mathematical Physics"),
    (4, "Computer Networks"),
    (4, "Fundamental Algorithms in C++"),
    (5, "Asymptotic Methods"),
    (4, "Introductory Internship"),
    (5, "Research Internship I"),
    (5, "Research Internship II"),
    (5, "Professional Internship"),
    (4, "Pre-Graduation Internship"),
    (4, "Bachelor's Thesis"),
]

total_courses = len(grades_main)
sum_grade_points = sum(grade for grade, _ in grades_main)
unweighted_gpa = sum_grade_points / total_courses

print(f"Total courses evaluated: {total_courses}")
print(f"Sum of grade points: {sum_grade_points}")
print(f"Cumulative Average (Unweighted): {unweighted_gpa:.2f}")

"""
Total courses evaluated: 33
Sum of grade points: 145
Cumulative Average (Unweighted): 4.39
"""
