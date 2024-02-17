import Map "mo:base/HashMap";
import Hash "mo:base/Hash";
import Nat "mo:base/Nat";
import Iter "mo:base/Iter";
import Text "mo:base/Text";
import Bool "mo:base/Bool";

// Todo project

actor Assistant {

  type Todo = {
    title : Text;
    description : Text;
    completed : Bool;
  };

  func natHash(n : Nat) : Hash.Hash {
    Text.hash(Nat.toText(n));
  };

  //let immutable -> rust mut
  //var  mutable

  var todos = Map.HashMap<Nat, Todo>(0, Nat.equal, natHash);
  var nextId : Nat = 0;

  //ID Todo atmaası

  public query func addTodo(description : Text, title : Text) : async Nat {
    let id = nextId;
    todos.put(id, { description = description; completed = false; title = title });
    nextId += 1;
    return id;
  };

  public func completeTodo(id : Nat) : async () {
    ignore do ? {
      let description = todos.get(id)!.description;
      let title = todos.get(id)!.title;
      todos.put(id, { description; completed = true; title });
    };
  };

  public query func showTodos() : async Text {
    var output : Text = "\n___TO-DOs___\n";
    for (todo : Todo in todos.vals()) {
      output #= "\n" # todo.description;
      if (todo.completed) {
        output #= "✔️";
      };
    };

    return output # "\n";
  };

  public func clearCopleted() : async () {
    todos := Map.mapFilter<Nat, Todo, Todo>(
      todos,
      Nat.equal,
      natHash,
      func(_, todo) { if (todo.completed) null else ?todo },
    );
  };

};
