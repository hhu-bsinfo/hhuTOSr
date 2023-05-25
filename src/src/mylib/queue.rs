
use std::cell::RefCell;
use std::rc::{Rc};
use std::fmt::Display;
use std::fmt;


// Definition eines generischen Listenelements
pub struct Node<T> {
    pub data: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

// Implementierung eines Konstruktors für ein generisches Listenelement 
impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

// Definition der generischen Liste
pub struct Queue<T> {
    head: Link<T>,
}

// Typ-Definition für eine Referenz auf ein Listenelement
pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T: PartialEq> Queue<T> {

   // Konstruktor, um eine leere Liste zu erzeugen
   pub const fn new() -> Self {
      Self { head: None }
   }
   
   // Ein Listenelement am Ende der Liste einfuegen   
   pub fn enqueue(&mut self, data: T) { 
      let new_node = Rc::new(RefCell::new(Node::new(data)));
      
      if self.head.is_none() {
         self.head = Some(new_node.clone());
      }
      else {
        let mut node = self.head.clone();
        while let Some(n) = node {
            if n.borrow_mut().next.is_none() {
	           n.borrow_mut().next = Some(new_node);
	           break;
            }
            node = n.borrow().next.clone();
        }
      }
    }
    
    // Das Listenelement am Kopf der Liste aushaengen und zurueckgeben
    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    self.head = Some(new_head);
                }
                None => {
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    // Suche und entferne das Element 'data'
    // Rueckgabewert: true:  falls das Element gefunden und geloescht wurde
    //                false: sonst
    pub fn remove(&mut self, data: T) -> bool {

       /* Hier muss Code eingefuegt werden */

   }

}


// Ausgabe der Liste
impl<T: Display> Display for Queue<T> {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}
