import random
from typing import List

class SubChapter:
  def __init__(self, name: str, samples_start: int | None, samples_end: int | None, exercises: int, skip = False):
    self.name = name
    self.samples_start = samples_start
    self.samples_end = samples_end
    self.exercises = exercises
    self.skip = skip
  
  def select_task(self):
    from_samples = self.samples_start != self.samples_end and random.choice([True, False])
    if from_samples:
      self.select_sample()
    else:
      self.select_exercise()

  def select_sample(self):
    print(f'Sample: {random.randint(self.samples_start, self.samples_end)}')

  def select_exercise(self):
    print(f'Exercise: {random.randrange(self.exercises) + 1}')


class Chapter:
  def __init__(self, name: str, sub_chapters: List[SubChapter], skip = False):
    self.name = name
    self.sub_chapters = sub_chapters
    self.skip = skip

chapters = [
  Chapter('Sampling and Descriptive Statistics', [
    SubChapter('1.1 Sampling', 1, 8, 9),
    SubChapter('1.2 Summary Statistics', 9, 16, 18),
    SubChapter('1.3 Graphical Summaries', 17, 18, 24),
    SubChapter('1. Supplementary Exercises', None, None, 19)
  ], True),
  Chapter('Probability', [
    SubChapter('2.1 Basic Ideas', 1, 9, 22),
    SubChapter('2.2 Counting Methods', 10, 16, 12),
    SubChapter('2.3 Conditional Probability and Independence', 17, 30, 39),
    SubChapter('2.4 Random Variables', 31, 47, 26),
    SubChapter('2.5 Linear Functions of Random Variables', 47, 51, 18),
    SubChapter('2.6 Jointly Distributed Random Variables', 52, 74, 33),
    SubChapter('2. Supplementary Exercises', None, None, 35)
  ]),
  Chapter('Propagation of Error', [
    SubChapter('3.1 Measurement Error', 1, 3, 9),
    SubChapter('3.2 Linear Combinations of Measurements', 4, 13, 20),
    SubChapter('3.3 Uncertainties for Functions of One Measurement', 14, 17, 20),
    SubChapter('3.4 Uncertainties for Functions of Several Measurements', 18, 28, 23),
    SubChapter('3. Supplementary Exercises', None, None, 23)
  ]),
  Chapter('Commonly Used Distributions', [
    SubChapter('4.1 The Bernoulli Distribution', 1, 3, 8),
    SubChapter('4.2 The Binomial Distribution', 5, 14, 26),
    SubChapter('4.3 The Poisson Distribution', 15, 27, 21),
    SubChapter('4.4 Some Other Discrete Distributions', 28, 37, 18),
    SubChapter('4.5 The Normal Distribution', 38, 51, 28),
    SubChapter('4.6 The Lognormal Distribution', 52, 55, 13),
    SubChapter('4.7 The Exponential Distribution', 56, 62, 15),
    SubChapter('4.8 Some Other Continuous Distributions', 63, 66, 17),
    SubChapter('4.9 Some Principles of Point Estimation', 67, 69, 10),
    SubChapter('4.10 Probability Plots', None, None, 10),
    SubChapter('4.11 The Central Limit Theorem ', 70, 76, 20),
    SubChapter('4.12 Simulation', 77, 85, 14),
    SubChapter('4. Supplementary Exercises', None, None, 30),
  ]),
  Chapter('Confidence Intervals', [
    SubChapter('5.1 Confidence Intervals for a Population Mean, Variance Known', 1, 10, 18),
    SubChapter('5.2 Confidence Intervals for a Population Mean, Variance Unknown', 11, 21, 36),
    SubChapter('5.3 Confidence Intervals for Proportions', 21, 24, 19),

  ])
]

actual_chapters = list(filter(lambda chapter: not chapter.skip, chapters))
selected_chapter = random.choice(actual_chapters)
print(selected_chapter.name)

actual_sub_chapters = list(filter(lambda chapter: not chapter.skip, selected_chapter.sub_chapters))
selected_sub_chapter = random.choice(actual_sub_chapters)
print(selected_sub_chapter.name)

selected_sub_chapter.select_task()
