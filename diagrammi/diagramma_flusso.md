# Titolo di primo livello
## Titolo di secondo livello
### Titolo di terzo livello

Testo normale
* Primo punto
* Secondo punto

In **grassetto**, in _corsivo_, in _**grassetto e corsivo**_.

```rust
fn main() {
    let a = 4;
}
```

```c
int main() {
    float a = 4.0;
}
```

```mermaid
%% start([ "" ])
%% input[/Input/]
%% processing[Processing]
%% output>Output]
%% decision{{Is Ok?}}
%% success([ "" ]); style success fill:#cfc
%% failure([ "" ]); style failure fill:#b00
flowchart TD
    start1([ "" ])
    input1[/Input/]
    processing1[Processing]
    output1>Output]
    decision1{{Is Ok?}}
    success1([ "" ]); style success1 fill:#cfc
    failure1([ "" ]); style failure1 fill:#b00
  
    start1-->input1
    input1-->processing1
    processing1-->output1
    output1-->decision1
    decision1--Yes-->success1
    decision1--No-->failure1
```
