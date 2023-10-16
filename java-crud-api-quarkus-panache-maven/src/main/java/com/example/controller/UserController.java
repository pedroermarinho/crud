package com.example.controller;

import com.example.forms.UserForm;
import com.example.services.UserService;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.*;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Response;

@Path("users")
@ApplicationScoped
@Transactional
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
public class UserController {


    @Inject
    private UserService userService;


    @POST
    public Response create(UserForm userForm){
        final var user = userService.create(userForm);
        return Response.status(Response.Status.CREATED).entity(user).build();
    }

    @PUT
    @Path("{id}")
    public Response update(@PathParam(value = "id") Long id, UserForm userForm){
        final var user = userService.update(id, userForm);
        return Response.ok().entity(user).build();
    }

    @GET
    public Response findAll(){
        final var users = userService.findAll();
        return Response.ok().entity(users).build();
    }

    @GET
    @Path("{id}")
    public Response findById(@PathParam(value = "id") Long id){
        final var userOpt = userService.findById(id);
        if (userOpt.isEmpty()){
            return Response.noContent().build();
        }
        return Response.ok().entity(userOpt.get()).build();
    }


    @DELETE
    @Path("{id}")
    public Response delete(@PathParam(value = "id")  Long id){
        userService.delete(id);
        return Response.ok().build();
    }

}
