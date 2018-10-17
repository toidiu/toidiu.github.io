---
title: "The Technical Interview in 2017"
nav: blog
id: blog
categories:
  - interview
tags:
  - Technical Interview
---

At the beginning of 2017, I was interviewing for junior back-end Scala positions at various types of companies including consultancy shops, startups and mature companies. The overall experience was pleasant and even fun, but there were also a lot more mishaps than I expected and a few that left a bad taste. However, I don't think the bad experiences were intentional but rather rooted in a lack of perspective/understanding that the company and candidate have about the other.

I spent 2016 working on my startup Wilfred (a Slack based chat-bot), which was trying to create and automate good workplace culture. I therefore spent a lot of time analyzing and trying to optimize a company's culture. With my unique insight into both the candidate and the company, I aim to speak about the priorities and perspectives of both.

Finally, the goal of this post is to help both the candidates and companies improve the process of Technical interviewing together. Despite it being 2017, I think the technical interview is still very lacking and can only be changed once expectations are set and mutually understood.

### The Interview Process
Most companies, with few exceptions, have a very similar hiring process. I will expand further on each step from the perspective of a candidate as well as the company.

- Application
    + Hiring service (Hired/recruiter)
    + Custom application on company site
- HR Screen
    + Questions for the candidate
    + Questions for the company representative
- Technical Screen
    + Live coding exercise (screen share)
    + Home coding challenge
- On-site
- Offer
    + Mutual understanding
- Rejection


### Application
The important task at this stage is for the company to give a timely response.

###### Company
Getting a timely response back to the candidate is key. A delay of one or two weeks could mean the candidate might be nearing the end of the interview process with other companies. 

Tools like Hired.com can help make the process seamless because they bridge the communicate gap. Additionally, such platforms record and provide information about role preferences, salary preferences, location preferences, availability and skills. These are important prerequisite factors which can help filter for the correct candidate and save time/money.

If a candidate is applying via the careers page on your site, then this is usually a good indicator of their interest. A timely response is key to attracting the applicant and also a way to make your company stand out (from personal experience, it usually takes over 1 week to get a response). 

Regardless of the application platform, it helps to have a designated person assigned to each candidate throughout the hiring process.

###### Candidate
From personal experience, the most effectiveness ways for finding companies are:

- use a hiring platform such as Hired.com
- companies which are involved/host relevant meetups
- find a recruiter who specializes in your area of interest


### HR Screen
Communication and mindset are the core qualities that should be vetted at this stage. 

###### Company
The candidate should be able to speak about his past experiences with clarity. Asking the candidate questions about challenges, conflicts and mistakes in past projects shows how involved they were, how they approach problems/work with others, and if they can communicate ideas.

Here are some questions to ask the candidate:

- Tell me about a project you worked on and challenges you faced?
- Tell me about a mistake you made and what you learned from it.
- Tell me about a tough bug and how you solved it.
- Tell me about when you had an conflict with a coworker and how you dealt with it.
- What is your ideal position?


###### Candidate
Since companies are generally testing your communication skills, the secret is to think about potential questions and write down the answers beforehand. The cool thing is that by doing this for a few sample questions, you will generally be able to answers a broader range of questions and speak about your projects in a more coherent manner.

> Note: Try to sound excited ('fake it' if necessary). Demonstrate that, if hired, it would be a pleasure to work with you. (I say 'fake it' because it's difficult to know what to be excited about if you don't know much about the company.)

This is also a good time to get insight into a company's values and role. Here are some questions to ask the company representative:

- What would be your ideal candidate?
- What is the % of time spent coding vs in meeting?
- What is the approx. size of the engineering team?
- What specific product would I be working on?
- What is the stack you are working with?


### Technical Screen
From the name, the goal at this stage is to vet the technical capabilities of the candidate. Two methods available are either `live coding (screen share)` or a `home challenge`.

###### Company
`Live coding` tests the candidate's **1)** ability to write code from scratch (no IDE), **2)** understanding of Computer Science, **3)** approach to problem solving, and **4)** ability to communicate. 

Because we are focusing on basic CS knowledge and working with limited time, the problems should generally be domain-independent. A general rule should be to ask about things that don't require internet access. This code will probably not be production ready.

`Home challenge` tests the candidates ability to **1)** come up with a working solution, **2)** organize code, **3)** write maintainable code, **4)** write tests. 

The coding assignment should ask the candidate to include design considerations, assumptions, and ideas on future improvements to gain further insight into their thought process. This is the place to ask domain specific questions and test aspects that might be critical for the role. 

A time limit is a great way to access the candidate's eagerness. However, the limit should be at least 3-7 days because a robust solution takes time to formulate, code, revise, test, and explain.

###### Candidate
Approach to a `live coding` technical screen:

- list/confirm assumptions
- list out possible approaches
- analyze their runtime
- pick the best/appropriate solution
- write pseudo-code
- write actual code
- test the code (sample input, most critical logic)

Approach to a `home challenge`:

- formulate a solution
- architect a design
- write code
- write tests!
- write maintainable code (revise design as needed)
- explain design considerations, assumptions, and future improvements
- code should aim to be production ready


### On-site
This process will often vary based on a company's priorities. While the company is the one dictating the schedule, I want to stress that this should not be a on-sided operation. A candidate should try to imagine the average work environment.

###### Company
The vetting criteria could include technical, behavioral, communication, culture fit, and problem solving. The exact structure of this stage will depend on the priorities of the company so I won't go into this much more.

Since the company is dictating the schedule for the on-site, it is responsible for considering and giving value to the candidate. Two things to consider are 

1. Communicating the schedule in detail
2. Ask the candidate what they needs to help make a decision (see code ...)

**1)** Below is a template of one of the most impressive schedules I received.

    Details:
    - Date: Wednesday October 21, 2015 7:28AM EST
    - Address: 100 Here St, NY
    - Please arrive by 9:50AM
    - Bring valid ID
    - Ask for Person 7 upon arrival

    Interview Roster:
    - 10:00am-11:00am EST: Person 1 (Senior Software Engineer)
    - 11:00am-12:00pm EST: Person 2 (Lead Engineer)
    - 12:00pm-1:00pm EST: Lunch w/ Person 3 (Software Engineer), Person 4 (Product)
    - 1:00pm-2:00pm EST: Person 5 (Director, Software Engineering)
    - 3:45pm-4:00pm EST: Person 6 (Recruiter)

    [Credit to iHeartRadio]

**2)** More impressively, it was followed by a call to explain the different people and ask what else I needed to help make a decision. In my case, I wanted to see a sample of the production code.


###### Candidate
There are two aspects to an on-site interview from the perspective of the candidate; technical assessment and company evaluation.

The `technical assessment` usually involves solving problems on a white-board, which can be difficult if not practiced. Especially difficult is organizing code and thoughts when solving a problem on a whiteboard. My advice is to outline the problem-solving steps and repeat those steps for each problem. Here are the steps that I follow and which share similarities with the technical screen process: **1)** note input/output **2)** note assumptions **3)** write pseudo-code **4)** write code **5)** test (via sample input).

> I have often gotten stuck when working on a whiteboard, not because I didn't know the solution but because after much writing and erasing, it became impossible to decipher and follow my own writing.

The `company evaluation` is often overlooked because its less concrete but I believe it is the most important of the two. The task is to imagine working at the company, alongside your potential co-workers. Here are a few things to help paint that picture:

- Ask people why they joined the company and why they stay. Here you should be listening for similar morals and sentiments.
- Ask to look at sample production code. It should look coherent, well tested, and like something you would want to work on.
- Ask to meet with the people you will report to. Try to judge if they show integrity and if they are someone you can trust.
- Ask what you can expect to work on in the next year.
- Ask about how decisions are made. Are they made top-down, bottom-up, random, merit-based.. this give a lot of insight into the company and its values.
- Assess the technical capabilities of your potential co-workers. Will you learn from them? 
- Assess the work space/environment.
- Assess the commute time.


### Offer

###### Company
I don't feel equipped to make general statements about this section because compensation and benefits will vary from company to company.

###### Candidate
Congrats! During the interview process you should have built up trust and understanding with the company. This means that if there is something you think is unfair with the final offer, reach out and communicate your concern. We tend to call this negotiating but I think that word has come to take on a negative connotation. I view it as reaching mutual understanding and finding a reasonable middle ground.

When deciding between different offers, simply list your priorities and choose the best fit. Some factors include compensation, perks, learning potential...


### Rejection

###### Company/Candidate 
It is vital to communicate the decision in a timely and respectful manner. In my opinion, ignoring to do this step is absolutely unacceptable, rude, and indicates bad character/culture of the candidate/company. As both parties should be aware, a lot of time and money are spent and to simply ignore or lead the other party on is bad practice.

Lastly, regardless of the outcome, both parties should view the interview process as building a new relation/network. Even if the other party is not currently a good fit, this can change in the future. A respectful and timely rejection is less likely to offend and keep the door ajar. If nothing else, realize that the tech world can be a small place and people talk.

