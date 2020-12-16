use crate::ticket::Ticket;
use crate::ticket_field::TicketField;
use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

mod ticket;
mod ticket_field;

fn main() -> Result<(), Box<dyn Error>> {
  let (my_ticket, tickets, mut ticket_fields) = parse_input()?;

  let bad_tickets = tickets
    .iter()
    .map(|ticket| {
      ticket
        .values
        .iter()
        .filter(|&&value| ticket_fields.iter().all(|f| !f.is_valid(value)))
        .collect_vec()
    })
    .collect_vec();

  let good_tickets = bad_tickets
    .iter()
    .positions(|t| t.is_empty())
    .map(|index| &tickets[index])
    .collect_vec();

  let mut found_fields = vec![];
  let num_fields = ticket_fields.len();

  for ticket_field in &mut ticket_fields {
    let mut possible = (0..num_fields)
      .filter(|i| !found_fields.contains(i))
      .collect_vec();
    for ticket in &good_tickets {
      let invalid = ticket
        .values
        .iter()
        .map(|value| ticket_field.is_valid(*value))
        .positions(|is_valid| !is_valid)
        .collect_vec();

      possible = possible
        .iter()
        .filter(|index| !invalid.contains(index))
        .map(|index| *index)
        .collect_vec();
      if possible.len() == 1 {
        found_fields.push(possible[0]);
        break;
      }
    }
    ticket_field.possible_indices = possible;
  }

  let mut completed_fields = vec![];
  
  while completed_fields.len() != num_fields {
    ticket_fields.sort_by_key(|ticket_field| ticket_field.possible_indices.len());
    ticket_fields.reverse();

    let mut first = ticket_fields.pop().unwrap();
    assert_eq!(first.possible_indices.len(), 1);

    first.index = Some(first.possible_indices[0] as u32);

    for ticket_field in &mut ticket_fields {
      ticket_field.possible_indices = ticket_field
        .possible_indices
        .iter()
        .filter(|&&index| index as u32 != first.index.unwrap())
        .map(|i| *i)
        .collect_vec();
    }

    completed_fields.push(first);
  }

  let part_one = bad_tickets.iter().flatten().map(|&&v| v).sum::<u32>();

  let part_two = completed_fields
    .iter()
    .filter(|field| field.identifier.contains("departure"))
    .map(|field| field.index.unwrap())
    .map(|index| my_ticket.values[index as usize] as u64)
    .fold(1, |acc, val| acc * val);

  println!("{:?}", part_one);
  println!("{:?}", part_two);

  Ok(())
}

fn parse_input() -> Result<(Ticket, Vec<Ticket>, Vec<TicketField>), std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let read = input
    .split(&format!("{}{}", LINE_ENDING, LINE_ENDING))
    .collect_vec();

  let ticket_fields = read[0]
    .split(LINE_ENDING)
    .map(TicketField::from_line)
    .collect_vec();

  let my_ticket = read[1]
    .split(LINE_ENDING)
    .skip(1)
    .map(Ticket::from_line)
    .collect_vec();
  let my_ticket = my_ticket.into_iter().nth(0).unwrap();

  let nearby_tickets = read[2]
    .split(LINE_ENDING)
    .skip(1)
    .map(Ticket::from_line)
    .collect_vec();

  Ok((my_ticket, nearby_tickets, ticket_fields))
}
