use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn registrar_funciona() {
	new_test_ext().execute_with(|| {
		// Debemos importar `assert_ok` de `frame_support`.
		// Se asegura que el valor sea una variante `Ok()`.
		assert_ok!(PiedraPapelTijera::registrar(RuntimeOrigin::signed(1)));
		let jugadores = PiedraPapelTijera::jugadores();
		assert_eq!(jugadores.len(), 1);
		assert_eq!(jugadores.first(), Some(&1));

		// System::assert_last_event(Event::Registrado { quien: 1 }.into());

		// Debemos importar `assert_noop` de `frame_support`.
		// Se asegura que el valor sea una variante `Err`,
		// y que el error sea el que se pasa como segundo parámetro.
		assert_noop!(PiedraPapelTijera::registrar(RuntimeOrigin::signed(1)), Error::<Test>::YaRegistrado);

		assert_ok!(PiedraPapelTijera::registrar(RuntimeOrigin::signed(2)));
		let jugadores = PiedraPapelTijera::jugadores(); // Recargar vector
		assert_eq!(jugadores.len(), 2);
		assert_eq!(jugadores.first(), Some(&1));
		assert_eq!(jugadores.last(), Some(&2));

		// System::assert_last_event(Event::Registrado { quien: 2 }.into());


		// assert_noop!(PiedraPapelTijera::registrar(RuntimeOrigin::signed(3)), Error::<Test>::JuegoLleno);
	});
}