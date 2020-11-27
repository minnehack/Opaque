# Functionality Spec

## Admin Interface
/admin

### Cosmetic Settings
/admin/cosmetic
- Primary Color
- Secondary Color
- Light/Dark theme
- Title Font
- Decorative Title Font (eg blade runner movie font)
- Text Font

### Schedule Management
/admin/schedule

The schedule is composed of "items" each of which are represented as a unit in the interface and contain:
- Check-In Phase
  - bool: deletable: false
  - datetime
  - selection: ["RSVP Yes Check-In", "Waitlist Check-In", "Walk-In Check-In"]
- Event Start
  - bool: deletable: false
  - datetime
- Kickoff:
  - bool: deletable: false
  - datetime
- Meal
  - bool: deletable: false (except "Midnight Snack")
  - datetime
  - selection: Meal Type: ["Lunch", "Dinner", "Breakfast", "Midnight Snack"]
  - long text: Food description (eg "Pizza")
  - short text: Vendor (do not display this field in the feed if == "")
  - long text: Dietary Restriction Information (displayed in feed as an expandable field)
- Workshop
  - datetime
  - short text: Workshop Name
  - long text: Workshop Description
  - duration: Workshop Duration
- Event End
  - datetime

Schedule is initialized with:
- RSVP Yes Check-In
- Waitlist Check-In
- Walk-In Check-In
- Event Start
- Kickoff
- Dinner
- Midnight Snack
- Other (text not editable) "Sleeping Room Opens"
- Other (text not editable) "Sleeping Room Closes"
- Breakfast
- Lunch
- Other (text not editable) "Hacking Ends"
```markdown
## Make sure you have your project submitted by ${eventParams.submissionDeadline}

If you do not have not submitted a project by ${eventParams.submissionDeadline} we cannot judge your project. If you are having an issue submitting your project, make sure you talk to one of the staff as soon as possible.
```
- Dinner
-

### Sponsor Management
/admin/sponsors

All sponsors that need access to something need to have an account in Opaque. These accounts can have certain flags set to indicate their role:
- `mentor`
  - Gives them the mentor role on discord (?)
  - Gives them a profile on the mentor page (?)
- `judge`
  - Gives them access to the judging interface during judging.
- `booth`
  - Nice for sponsor chicken.
- `contact`
  - Gives access to resumes when they are released.

You can set the resumeAccess flag on any mentor, judge, sponsor account to give that person access to the resumes when uploads are disabled. Resume (re)upload is disabled at the start of the event, and resume-flagged sponsors get access to the resumes at the same time.

### Prize Management
/admin/prizes


## Website
/

### Sponsors
/sponsors

### Schedule
/schedule

### FAQ
/faq

### Code of Conduct
/codeofconduct

### Rules
/rules

### Prompt
/prompt
_Must be logged in, registered, checked in, and the event must have started to view this page._

## Registration
/register

### Mentor/Sponsor/Judge Registration
/msjregister

We will give some sponsor accounts access to the resumes. Resume (re)upload is disabled at the start of the event, and resume-flagged sponsors get access to the resumes at the same time.

## RSVP (4-stop)
/4stop

## Chicken (Check-In)
/chicken

## Feed
/feed

Announcements, events, updates, deadlines,

### Projector
/feed/projector

Controlled by the admin interface, displays:
- Welcome message at the beginning
- Opening slide deck
-

## ~~Mentor Matcher~~ (maybe in the future)
/mentee & /mentor

## Submissions
/submit

### Submission Form
- short text: Group Name
- tag field: group members, only checked-in emails will form a tag
- Button at bottom that says "Complete Submission" with a note below saying that this will make the submission public but you can update the submission until the deadline.

## Gallery
/gallery
_No login required._

All of the submissions from the current event that have been completed.

### Voting
/gallery
_You can only vote during the judging period._

## Judging
/judge

## ~~Awards Presentation~~ (?)

## Past
/past

A catalogue of past events, each event containing:
- Winners
- Photo Gallery
- Submissions

### Hall of Fame
/past/halloffame

Winners of all past events

### Photo Gallery
/past/${event}/photos

### Winners
/past/${event}/winners


## Notes

`eventParams` is an object generated from the event settings and must be regenerated when anything on the admin page is changed. It must be filtered of sensetive information because it will be sent to all clients.


# Tech

## UI

There are two categories of pages in the Opaque interface. Static pages generated from event props such as the home page and the schedule and dynamic (interactive) pages which can be as simple as displaying different content depending on whether the user is logged in or as complex as the admin interface. The static pages will not use
