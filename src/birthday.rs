use chrono::{Datelike, NaiveDate};
use ics::properties::{Categories, Description, DtEnd, DtStart, Summary, Transp, Trigger};
use ics::{Alarm, Event};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Birthday {
    pub name: String,
    pub date: NaiveDate,
}

impl<'a> Birthday {
    pub fn into_event(self, year: i32, dtstamp: String) -> Event<'a> {
        let mut event = Event::new(Uuid::new_v4().hyphenated().to_string(), dtstamp);
        if self.date.year() == 0 {
            event.push(Summary::new(self.name));
        } else {
            event.push(Summary::new(format!(
                "{} ({})",
                self.name,
                year - self.date.year()
            )));
        }
        event.push(DtStart::new(date_to_ical_string(
            &self.date.with_year(year).unwrap(),
        )));
        event.push(DtEnd::new(date_to_ical_string(
            &self.date.with_year(year).unwrap().succ_opt().unwrap(),
        )));
        event.push(Categories::new("Geburtstag"));
        event.push(Transp::new("TRANSPARENT"));

        let alarm = Alarm::display(Trigger::new("PT0H"), Description::new("Geburtstag"));
        event.add_alarm(alarm);

        event
    }
}

fn date_to_ical_string(date: &NaiveDate) -> String {
    date.format("%Y%m%d").to_string()
}
