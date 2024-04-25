use std::io;

fn convert_to_int(str_valor: & String) -> i32
{
    let int_valor: i32 = str_valor.trim().parse::<i32>().unwrap();
    int_valor
}

fn main()
{
    let mut quantidade_alunos = String::new();
    let mut reprovados: i32;
    let mut recuperacao: i32;
    let mut aprovados: i32;
    let mut contador: i32;

    println!("Digite o numero de alunos: ");
    io::stdin().read_line(&mut quantidade_alunos).expect("quantidade_alunos: valor inválido!");
    
    reprovados = 0;
    recuperacao = 0;
    aprovados = 0;
    contador = 0;

    while contador < convert_to_int(&quantidade_alunos)
    {
        contador += 1;
        
        println!("Digite a média do aluno {} de {}: ", contador, quantidade_alunos);

        let mut media_aluno = String::new();
        io::stdin().read_line(&mut media_aluno).expect("media_aluno: valor inválido!");

        if convert_to_int(&media_aluno) < 3
        {
            reprovados += 1;
        }
        else if convert_to_int(&media_aluno) >= 3 && convert_to_int(&media_aluno) < 6
        {
            recuperacao += 1;
        }
        else if convert_to_int(&media_aluno) >= 6
        {
            aprovados += 1;
        }
    }
    
    println!("Alunos aprovados: {}\nAlunos em recuperação: {}\nAlunos reprovados: {}", aprovados, recuperacao, reprovados);

    // let mut medias = String::new();
    // io::stdin().read_line(&mut medias).expect("Erro ao ler medias");
    // let mut soma_rec = 0;
    // let mut i = 0;
    // while convert_to_int(&medias) > i
    // {
    //     let mut media_aluno = String::new();
    //     io::stdin().read_line(&mut media_aluno).expect("Erro ao ler medias");
    //     i += 1;
    //     if convert_to_int(&media_aluno) >= 3 && convert_to_int(&media_aluno) < 6
    //     {
    //         soma_rec += 1;
    //     }
    // }
    // println!("Alunos em recuperação: {}", soma_rec);
}
