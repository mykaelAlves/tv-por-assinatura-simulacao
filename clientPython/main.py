import requests
import json
import os

API_BASE_URL = 'http://127.0.0.1:9999'

def limpar_tela():
    """Limpa o console do terminal."""
    os.system('cls' if os.name == 'nt' else 'clear')

def get_client_data():
    """Solicita os dados do cliente (ID e Nome) no terminal."""
    try:
        id_input = input("Digite o ID do cliente: ")
        nome = input("Digite o Nome do cliente: ")

        if not id_input or not nome:
            print("\nErro: ID e Nome são obrigatórios.")
            return None
            
        cliente_id = int(id_input)
        return {"id": cliente_id, "nome": nome}
        
    except ValueError:
        print("\nErro: O ID do cliente deve ser um número inteiro.")
        return None

def display_result(status, data):
    """Exibe o resultado da requisição de forma formatada."""
    print("\n--- Resultado da Operação ---")
    # Formata o dicionário Python em uma string JSON legível
    pretty_data = json.dumps(data, indent=2, ensure_ascii=False)
    print(f"Status: {status}\n")
    print(f"Resposta:\n{pretty_data}")
    print("----------------------------")

async def make_api_request(endpoint, method):
    """
    Coleta os dados do cliente e envia uma requisição para a API.
    
    Args:
        endpoint (str): O endpoint da API a ser chamado (ex: '/api/getPlano').
        method (str): O método HTTP a ser usado ('POST', 'PUT', 'DELETE').
    """
    cliente = get_client_data()
    if not cliente:
        return 

    url = f"{API_BASE_URL}{endpoint}"
    headers = {'Content-Type': 'application/json'}

    print(f"\nEnviando requisição {method} para {url}...")

    try:
        response = requests.request(
            method=method, 
            url=url, 
            headers=headers, 
            json=cliente,
            timeout=5 
        )
        
        try:
            result_data = response.json()
        except json.JSONDecodeError:
            result_data = {"erro": "A resposta não é um JSON válido.", "conteudo": response.text}

        display_result(response.status_code, result_data)

    except requests.exceptions.RequestException as e:
        error_data = {
            "mensagem": "Falha na comunicação com o servidor. Verifique se ele está no ar.",
            "detalhes": str(e)
        }
        display_result("Erro de Conexão", error_data)

def exibir_menu():
    """Exibe o menu de opções para o usuário."""
    print(" --- Cliente da API de Planos --- ")
    print("\n-- Assinaturas --")
    print("1. Assinar Plano Básico")
    print("2. Assinar Plano Premium")
    print("3. Assinar Plano Ultra")
    print("\n-- Operações --")
    print("4. Consultar Plano do Cliente")
    print("5. Melhorar Plano (Upgrade)")
    print("6. Rebaixar Plano (Downgrade)")
    print("7. Cancelar Plano")
    print("\n8. Sair")
    return input("\nEscolha uma opção: ")

async def main():
    """Função principal que gerencia o loop do menu."""
    while True:
        limpar_tela()
        escolha = exibir_menu()

        if escolha == '1':
            await make_api_request('/api/assinarPlano/2', 'POST')
        elif escolha == '2':
            await make_api_request('/api/assinarPlano/1', 'POST')
        elif escolha == '3':
            await make_api_request('/api/assinarPlano/0', 'POST')
        elif escolha == '4':
            await make_api_request('/api/getPlano', 'POST')
        elif escolha == '5':
            await make_api_request('/api/melhorarPlano', 'PUT')
        elif escolha == '6':
            await make_api_request('/api/rebaixarPlano', 'PUT')
        elif escolha == '7':
            confirmacao = input("Tem certeza que deseja cancelar? (s/n): ").lower()
            if confirmacao == 's':
                await make_api_request('/api/cancelarPlano', 'DELETE')
            else:
                print("\nOperação cancelada.")
        elif escolha == '8':
            print("Adios!")
            break
        else:
            print("\nOpção inválida. Tente novamente.")

        input("\nPressione Enter para continuar...")


if __name__ == "__main__":
    import asyncio

    asyncio.run(main())