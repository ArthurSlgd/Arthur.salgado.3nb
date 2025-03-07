use std::ptr;

/// Estrutura de um nó da fila, armazenando um valor e um ponteiro para o próximo nó.
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

/// Estrutura da fila propriamente dita.
pub struct Queue<T> {
    front: Option<Box<Node<T>>>, // Ponteiro para o primeiro elemento
    rear: *mut Node<T>,         // Ponteiro cru para o último elemento
    size: usize,                // Contador de elementos
}

impl<T> Queue<T> {
    /// Cria uma nova fila vazia.
    pub fn new() -> Self {
        Queue {
            front: None,
            rear: ptr::null_mut(),
            size: 0,
        }
    }

    /// Adiciona um elemento ao final da fila.
    pub fn enqueue(&mut self, elem: T) {
        let mut new_node = Box::new(Node {
            value: elem,
            next: None,
        });

        let raw_ptr: *mut _ = &mut *new_node;
        
        if self.rear.is_null() {
            // Se a fila está vazia, front e rear devem apontar para o mesmo nó.
            self.front = Some(new_node);
        } else {
            // Unsafe necessário para manipular ponteiro cru corretamente
            unsafe { (*self.rear).next = Some(new_node); }
        }
        self.rear = raw_ptr;
        self.size += 1;
    }

    /// Remove e retorna o elemento do início da fila.
    pub fn dequeue(&mut self) -> Option<T> {
        self.front.take().map(|mut front_node| {
            self.front = front_node.next.take();

            if self.front.is_none() {
                self.rear = ptr::null_mut();
            }

            self.size -= 1;
            front_node.value
        })
    }

    /// Retorna uma referência ao elemento da frente da fila, sem removê-lo.
    pub fn peek(&self) -> Option<&T> {
        self.front.as_ref().map(|node| &node.value)
    }

    /// Retorna o tamanho atual da fila.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Retorna true se a fila estiver vazia.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {} // Remove todos os elementos para liberar memória corretamente
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_peek() {
        let mut queue = Queue::new();
        queue.enqueue(42);
        assert_eq!(queue.peek(), Some(&42));
        assert_eq!(queue.dequeue(), Some(42));
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn test_len() {
        let mut queue = Queue::new();
        assert_eq!(queue.len(), 0);
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.len(), 2);
        queue.dequeue();
        assert_eq!(queue.len(), 1);
        queue.dequeue();
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());
        queue.enqueue(10);
        assert!(!queue.is_empty());
        queue.dequeue();
        assert!(queue.is_empty());
    }
}