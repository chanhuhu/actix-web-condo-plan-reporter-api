-- Add migration script here
CREATE VIEW project_report_view AS
SELECT p.id,
       p.name        project_name,
       fp.name       floor_plan_name,
       fp.image_url,
       i.name        issue_name,
       i.description issue_description,
       i.location    issue_location,
       f.name        filename,
       f.url         file_url
FROM projects p
         JOIN floor_plans fp on p.id = fp.project_id
         JOIN issues i on fp.id = i.floor_plan_id
         JOIN files f on i.id = f.issue_id

