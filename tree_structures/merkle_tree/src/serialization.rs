use std::io::{Cursor, Read};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::merkle_tree::MerkleTree;
use crate::node::Node;

fn serialize_node(node: &Option<Box<Node>>, buf: &mut Vec<u8>) {
    match node {
        Some(node) => {
            buf.write_u8(1).unwrap(); // Indicates presence of a node
            buf.write_u32::<BigEndian>(node.value.len() as u32).unwrap();
            buf.extend_from_slice(&node.value);
            buf.write_u8(node.some_child_empty as u8).unwrap();
            buf.write_i64::<BigEndian>(node.height).unwrap();
            serialize_node(&node.left, buf);
            serialize_node(&node.right, buf);
        }
        None => {
            buf.write_u8(0).unwrap(); // Indicates absence of a node
        }
    }
}

pub fn serialize_tree(tree: &MerkleTree) -> Vec<u8> {
    let mut buf = Vec::new();
    serialize_node(&tree.root, &mut buf);
    buf
}

// Deserialize a Node recursively
fn deserialize_node(cursor: &mut Cursor<&[u8]>) -> Option<Box<Node>> {
    if cursor.read_u8().unwrap() == 1 {
        let value_len = cursor.read_u32::<BigEndian>().unwrap() as usize;
        let mut value = vec![0u8; value_len];
        cursor.read_exact(&mut value).unwrap();
        let some_child_empty = cursor.read_u8().unwrap() != 0;
        let height = cursor.read_i64::<BigEndian>().unwrap();
        let left = deserialize_node(cursor);
        let right = deserialize_node(cursor);
        Some(Box::new(Node {
            value,
            left,
            right,
            some_child_empty,
            height,
        }))
    } else {
        None
    }
}

pub fn deserialize_tree(data: &[u8]) -> MerkleTree {
    let mut cursor = Cursor::new(data);
    MerkleTree {
        root: deserialize_node(&mut cursor),
    }
}
