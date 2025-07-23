document.addEventListener('DOMContentLoaded', () => {
    const API_BASE_URL = 'http://127.0.0.1:9999';
    const resultDisplay = document.getElementById('result-display');

    function getClientData() {
        const idInput = document.getElementById('id').value;
        const nome = document.getElementById('nome').value;

        if (!idInput || !nome) {
            alert('Por favor, preencha o ID e o Nome do cliente.');
            return null;
        }
        
        const id = parseInt(idInput, 10);
        if (isNaN(id)) {
            alert('O ID do cliente deve ser um número.');
            return null;
        }

        return { id, nome };
    }

    function displayResult(status, data) {
        const prettyData = JSON.stringify(data, null, 2);
        resultDisplay.textContent = `Status: ${status}\n\nResposta:\n${prettyData}`;
    }

    async function makeApiRequest(endpoint, method) {
        const cliente = getClientData();
        if (!cliente) return;

        try {
            const response = await fetch(`${API_BASE_URL}${endpoint}`, {
                method: method,
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(cliente),
            });

            const result = await response.json();
            displayResult(response.status, result);

        } catch (error) {
            displayResult('Erro de Conexão', { 
                mensagem: 'Falha na comunicação com o servidor. Verifique se ele está no ar.',
                detalhes: error.message 
            });
        }
    }

    document.getElementById('btn-assinar-basico').addEventListener('click', () => makeApiRequest('/api/assinarPlano/2', 'POST'));
    document.getElementById('btn-assinar-premium').addEventListener('click', () => makeApiRequest('/api/assinarPlano/1', 'POST'));
    document.getElementById('btn-assinar-ultra').addEventListener('click', () => makeApiRequest('/api/assinarPlano/0', 'POST'));
    
    document.getElementById('btn-get-plano').addEventListener('click', () => makeApiRequest('/api/getPlano', 'POST'));
    document.getElementById('btn-melhorar-plano').addEventListener('click', () => makeApiRequest('/api/melhorarPlano', 'PUT'));
    document.getElementById('btn-rebaixar-plano').addEventListener('click', () => makeApiRequest('/api/rebaixarPlano', 'PUT'));
    
    document.getElementById('btn-cancelar-plano').addEventListener('click', () => {
        if (confirm('Tem certeza que deseja cancelar sua assinatura?')) {
            makeApiRequest('/api/cancelarPlano', 'DELETE');
        }
    });
});