/*
 * Adapter
 */

#include <iostream>
#include <string>

typedef int Cable; // cable con electrones

/* Adaptee interfaz fuente */
class InterfazTerminalEuropea
{
public:
virtual int voltaje() = 0;

virtual Cable vivo() = 0;
virtual Cable neutro() = 0;
virtual Cable tierra() = 0;
};

/* Adaptee */
class Terminal : public InterfazTerminalEuropea
{
public:
int voltaje() { return 220; }

Cable vivo() { return 1; }
Cable neutro() { return -1; }
Cable tierra() { return 0; }
};

/* Interfaz Target */
class InterfazTerminal
{
public:
virtual int voltaje() = 0;
virtual Cable vivo() = 0;
virtual Cable neutro() = 0;
};

/* El Adapter */
class Adapter : public InterfazTerminal
{
InterfazTerminalEuropea* Terminal;

public:
void enchufar(InterfazTerminalEuropea* salida)
{
Terminal = salida;
}

int voltaje() { return 110; }
Cable vivo() { return Terminal->vivo(); }
Cable neutro() { return Terminal->neutro(); }
};

/* Client */
class CoffeMaker
{
InterfazTerminalTerminal* energia;

public:
void enchufar(InterfazTerminal* fuentedevoltaje)
{
energia = fuentedevoltaje;
}

void boil()
{
if (energia->voltaje() > 110)
{
std::cout << "CoffeMaker está llamas!" << std::endl;
return;
}

if (energia->vivo() == 1 && energia->neutro() == -1)
{
std::cout << "Tiempo de café!" << std::endl;
}
}
};


int main()
{
Terminal* terminal = new Terminal;
Adapter* adapter = new Adapter;
CoffeMaker* cafetera = new CoffeMaker;

/* Enchufar */
adapter->enchufar(terminal);
cafetera->enchufar(adapter);

/* Having coffee */
cafetera->boil();

return 0;
}