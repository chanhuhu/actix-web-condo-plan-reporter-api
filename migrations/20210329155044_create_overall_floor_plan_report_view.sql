-- Add migration script here
CREATE VIEW floor_plan_report_view AS
SELECT fp.id,
       fp.image_url,
       fp.name       floor_plan_name,
       i.name        issue_name,
       i.description issue_description,
       i.location    issue_location,
       f.name        filename,
       f.url         file_url
FROM floor_plans fp
         JOIN issues i on fp.id = i.floor_plan_id
         JOIN files f on i.id = f.issue_id
