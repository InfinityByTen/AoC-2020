#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <optional>

std::vector< int32_t >
get_puzzle( )
{
    std::ifstream file( "./input_1_d1.txt" );
    std::vector< int32_t > puzzle;
    std::string line;
    while ( std::getline( file, line ) )
    {
        std::istringstream iss( line );
        int32_t num;
        if ( !( iss >> num ) )
        {
            std::cerr << "Failure to read number!! \n";
            break;
        }
        else
        {
            puzzle.push_back( num );
        }
    }
    return puzzle;
}

std::optional< std::pair< int32_t, int32_t > >
get_components( const std::vector< int32_t >& puzzle, int32_t sum )
{
    std::vector< int32_t > missing_pieces;
    std::for_each( puzzle.begin( ), puzzle.end( ), [&missing_pieces, &sum]( auto entry ) {
        missing_pieces.push_back( sum - entry );
    } );

    for ( int i = 0; i < puzzle.size( ); ++i )
    {
        if ( std::binary_search( puzzle.begin( ), puzzle.end( ), missing_pieces[ i ] ) )
        {
            return std::make_pair( puzzle[ i ], missing_pieces[ i ] );
        }
    }
    return std::nullopt;
}

void
solve_part_1( const std::vector< int32_t >& puzzle )
{
    auto result = get_components( puzzle, 2020 );
    if ( result )
    {
        std::cerr << "num: " << result->first << " missing_piece: " << result->second
                  << " prod: " << result->first * result->second << std::endl;
    }
}

// This is still not elegant enough. But whatever.
void
solve_part_2( const std::vector< int32_t >& puzzle )
{
    std::vector< int32_t > missing_pieces;
    std::for_each( puzzle.begin( ), puzzle.end( ),
                   [&missing_pieces]( auto entry ) { missing_pieces.push_back( 2020 - entry ); } );

    for ( int i = 0; i < puzzle.size( ); ++i )
    {
        auto components = get_components( puzzle, missing_pieces[ i ] );
        if ( components )
        {
            std::cerr << "numbers are: " << puzzle[ i ] << ", " << components->first << ", "
                      << components->second
                      << ", product:" << puzzle[ i ] * components->first * components->second
                      << std::endl;
            return;
        }
    }
}

int
main( int argc, char const* argv[] )
{
    auto puzzle = get_puzzle( );

    // to use binary search to do finds later on.
    std::sort( puzzle.begin( ), puzzle.end( ) );

    solve_part_1( puzzle );

    // Apparently, the input was same for both parts. I was expecting a bigger challenge. Too used
    // to Code Jam.
    solve_part_2( puzzle );

    return 0;
}